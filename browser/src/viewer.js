import loadMujoco from "@mujoco/mujoco";
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";

const PLANE = 0, SPHERE = 2, CAPSULE = 3, ELLIPSOID = 4, CYLINDER = 5, BOX = 6, MESH = 7;
const COLLISION_GROUP = 3;

export async function createViewer(canvas, { xml, meshes }) {
  const mj = await loadMujoco();
  const vfs = new mj.MjVFS();
  for (const [name, bytes] of Object.entries(meshes)) vfs.addBuffer(name, bytes);
  const model = mj.MjModel.from_xml_string(xml, vfs);
  vfs.delete();
  const data = new mj.MjData(model);
  mj.mj_kinematics(model, data);

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x888888, 2));
  const sun = new THREE.DirectionalLight(0xffffff, 2);
  sun.position.set(1, -1, 3);
  scene.add(sun);

  const objects = buildGeoms(model);
  for (const obj of objects) if (obj) scene.add(obj);

  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setPixelRatio(devicePixelRatio);
  const camera = new THREE.PerspectiveCamera(45, 1, 0.01, 100);
  camera.up.set(0, 0, 1);
  const controls = new OrbitControls(camera, canvas);

  let qpos = null;
  const update = () => {
    if (qpos) {
      data.qpos.set(qpos);
      mj.mj_kinematics(model, data);
      qpos = null;
    }
    const xpos = data.geom_xpos;
    const xmat = data.geom_xmat;
    objects.forEach((obj, i) => {
      if (!obj) return;
      const p = i * 3, r = i * 9;
      obj.matrix.set(
        xmat[r], xmat[r + 1], xmat[r + 2], xpos[p],
        xmat[r + 3], xmat[r + 4], xmat[r + 5], xpos[p + 1],
        xmat[r + 6], xmat[r + 7], xmat[r + 8], xpos[p + 2],
        0, 0, 0, 1,
      );
      obj.matrixWorldNeedsUpdate = true;
    });
  };

  update();
  frame(scene, camera, controls, objects);

  const resize = () => {
    const { clientWidth: w, clientHeight: h } = canvas;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  };
  new ResizeObserver(resize).observe(canvas);
  resize();

  renderer.setAnimationLoop(() => {
    update();
    controls.update();
    renderer.render(scene, camera);
  });

  return {
    setQpos(next) {
      qpos = next;
    },
  };
}

function buildGeoms(model) {
  const materials = new Map();
  const objects = [];
  for (let i = 0; i < model.ngeom; i++) {
    if (model.geom_group[i] === COLLISION_GROUP) {
      objects.push(null);
      continue;
    }
    const geometry = geomGeometry(model, i);
    if (!geometry) {
      objects.push(null);
      continue;
    }
    const mesh = new THREE.Mesh(geometry, material(model, i, materials));
    mesh.matrixAutoUpdate = false;
    objects.push(mesh);
  }
  return objects;
}

function geomGeometry(model, i) {
  const s = model.geom_size.slice(i * 3, i * 3 + 3);
  switch (model.geom_type[i]) {
    case PLANE:
      return new THREE.PlaneGeometry(2 * (s[0] || 5), 2 * (s[1] || 5));
    case SPHERE:
      return new THREE.SphereGeometry(s[0], 24, 16);
    case ELLIPSOID:
      return new THREE.SphereGeometry(1, 24, 16).scale(s[0], s[1], s[2]);
    case CAPSULE:
      return new THREE.CapsuleGeometry(s[0], 2 * s[1], 8, 24).rotateX(Math.PI / 2);
    case CYLINDER:
      return new THREE.CylinderGeometry(s[0], s[0], 2 * s[1], 32).rotateX(Math.PI / 2);
    case BOX:
      return new THREE.BoxGeometry(2 * s[0], 2 * s[1], 2 * s[2]);
    case MESH:
      return meshGeometry(model, model.geom_dataid[i]);
    default:
      return null; // ponytail: hfield, sdf and flex are not drawn, add them when a lab uses them
  }
}

function meshGeometry(model, id) {
  const v = model.mesh_vertadr[id], nv = model.mesh_vertnum[id];
  const f = model.mesh_faceadr[id], nf = model.mesh_facenum[id];
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute(
    "position",
    new THREE.BufferAttribute(model.mesh_vert.slice(v * 3, (v + nv) * 3), 3),
  );
  const faces = new Uint32Array(model.mesh_face.subarray(f * 3, (f + nf) * 3));
  geometry.setIndex(new THREE.BufferAttribute(faces, 1));
  geometry.computeVertexNormals();
  return geometry;
}

function material(model, i, cache) {
  const matid = model.geom_matid[i];
  const rgba = matid >= 0
    ? model.mat_rgba.slice(matid * 4, matid * 4 + 4)
    : model.geom_rgba.slice(i * 4, i * 4 + 4);
  const key = rgba.join();
  if (!cache.has(key)) {
    cache.set(key, new THREE.MeshStandardMaterial({
      color: new THREE.Color(rgba[0], rgba[1], rgba[2]),
      opacity: rgba[3],
      transparent: rgba[3] < 1,
      side: THREE.DoubleSide,
    }));
  }
  return cache.get(key);
}

function frame(scene, camera, controls, objects) {
  const box = new THREE.Box3();
  for (const obj of objects) {
    if (!obj || obj.geometry.type === "PlaneGeometry") continue;
    obj.updateMatrixWorld();
    box.expandByObject(obj);
  }
  const center = box.getCenter(new THREE.Vector3());
  const size = box.getSize(new THREE.Vector3()).length();
  controls.target.copy(center);
  camera.position.copy(center).add(new THREE.Vector3(size, -size, size * 0.7));
}
