// Live 3D for the Projects hall. The build on show turns on the turntable, smooth and sharp at
// any size, in the brand kit's matte light (the same hemisphere and sun as its renderer), and a
// drag spins it. The small models in the select strip turn the same way while pointed at or on
// show. All of it is optional: without this module, or without WebGL, the flipbook strips stay,
// and they still turn.
import * as THREE from './vendor/three.module.min.js';

const hall = document.querySelector('.hall');
if (hall) start().catch(() => {});

async function start() {
  const calm = () => {
    const m = document.documentElement.dataset.motion;
    return m === 'reduce' || (m !== 'full' && matchMedia('(prefers-reduced-motion: reduce)').matches);
  };
  const stage = (w, h) => {
    const r = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    r.setClearColor(0x000000, 0);
    r.shadowMap.enabled = true;
    r.shadowMap.type = THREE.PCFSoftShadowMap;
    r.setSize(w, h, false);
    return r;
  };
  let big, small;
  try { big = stage(2, 2); small = stage(96, 96); } catch { return; }

  // The kit's light: a true-isometric camera, a hemisphere and a sun solved so a white cube
  // lands on its three faces (tools/library-render.html in the brand kit).
  const scene = new THREE.Scene();
  const sun = new THREE.DirectionalLight(0xffffff, 1.76);
  sun.castShadow = true;
  sun.shadow.mapSize.set(1024, 1024);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x888888, 1.8), sun, sun.target);
  const VIEW = new THREE.Vector3(1, 1, 1).normalize(), SUN = new THREE.Vector3(0.21, 0.76, 0.61).normalize();
  const RIGHT = new THREE.Vector3(1, 0, -1).normalize(), UP = new THREE.Vector3().crossVectors(VIEW, RIGHT).negate().normalize();
  const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.01, 100);

  // Each model turns about its own centre, framed once for its widest view so it never
  // changes size as it turns.
  const loader = new THREE.ObjectLoader(), cache = new Map();
  const fit = (obj) => {
    obj.traverse((o) => { if (o.isMesh) o.castShadow = o.receiveShadow = true; });
    const pivot = new THREE.Group();
    obj.position.sub(new THREE.Box3().setFromObject(obj).getCenter(new THREE.Vector3()));
    pivot.add(obj);
    const v = new THREE.Vector3();
    let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity, far = 0;
    for (let k = 0; k < 24; k++) {
      pivot.rotation.y = (k * Math.PI) / 12;
      pivot.updateMatrixWorld(true);
      obj.traverse((o) => {
        if (!o.isMesh) return;
        const a = o.geometry.attributes.position;
        for (let i = 0; i < a.count; i++) {
          v.fromBufferAttribute(a, i).applyMatrix4(o.matrixWorld);
          far = Math.max(far, v.length());
          const x = v.dot(RIGHT), y = v.dot(UP);
          x0 = Math.min(x0, x); x1 = Math.max(x1, x); y0 = Math.min(y0, y); y1 = Math.max(y1, y);
        }
      });
    }
    return { pivot, half: (Math.max(x1 - x0, y1 - y0) / 2) * 1.06, cx: (x0 + x1) / 2, cy: (y0 + y1) / 2, far };
  };
  const load = (url) => {
    if (!cache.has(url)) cache.set(url, fetch(url).then((r) => r.json()).then((j) => fit(loader.parse(j))));
    return cache.get(url);
  };
  // Clockwise seen from above is a negative turn about y.
  const draw = (r, m, angle) => {
    const aim = new THREE.Vector3().addScaledVector(RIGHT, m.cx).addScaledVector(UP, m.cy);
    camera.left = camera.bottom = -m.half;
    camera.right = camera.top = m.half;
    camera.far = m.far * 8 + 10;
    camera.position.copy(aim).addScaledVector(VIEW, m.far * 4 + 5);
    camera.lookAt(aim);
    camera.updateProjectionMatrix();
    sun.position.copy(SUN).multiplyScalar(m.far * 3 + 2);
    const sc = sun.shadow.camera;
    sc.left = sc.bottom = -m.far * 1.2; sc.right = sc.top = m.far * 1.2; sc.far = m.far * 6 + 4;
    sc.updateProjectionMatrix();
    sun.shadow.bias = -0.0006; sun.shadow.normalBias = m.far * 0.006;
    m.pivot.rotation.y = angle;
    scene.add(m.pivot);
    r.render(scene, camera);
    scene.remove(m.pivot);
  };

  // The turntable: one canvas, moved to whichever build is on show.
  const canvas = big.domElement;
  canvas.className = 'turntable-3d';
  canvas.setAttribute('aria-hidden', 'true');
  let on = null, angle = 0, drag = null, seen = true;
  const size = () => {
    const box = canvas.getBoundingClientRect(), dpr = Math.min(devicePixelRatio || 1, 2);
    if (box.width) big.setSize(Math.round(box.width * dpr), Math.round(box.height * dpr), false);
  };
  new ResizeObserver(size).observe(canvas);
  const mount = async (slug) => {
    const table = document.querySelector('#show-' + slug + ' .turntable');
    if (!table || !table.dataset.model) return;
    table.appendChild(canvas);
    size();
    const m = await load(table.dataset.model).catch(() => null);
    if (!m || table !== canvas.parentNode) return;
    table.classList.add('live');
    on = m;
    draw(big, on, angle);
  };
  hall.addEventListener('show', (e) => mount(e.detail));
  const first = hall.querySelector('.show:not([hidden])');
  if (first) mount(first.id.slice(5));
  canvas.addEventListener('pointerdown', (e) => { drag = e.clientX; canvas.setPointerCapture(e.pointerId); canvas.classList.add('dragging'); });
  canvas.addEventListener('pointermove', (e) => {
    if (drag === null || !on) return;
    angle += (e.clientX - drag) * 0.012;
    drag = e.clientX;
    draw(big, on, angle);
  });
  const release = () => { drag = null; canvas.classList.remove('dragging'); };
  canvas.addEventListener('pointerup', release);
  canvas.addEventListener('pointercancel', release);
  new IntersectionObserver((es) => { seen = es[0].isIntersecting; }).observe(hall);

  // The select strip: each tile's model drawn sharp once, then turning while it's pointed at
  // or on show. One small renderer draws for all of them.
  const tiles = [...hall.querySelectorAll('.tile[data-model]:not([data-model=""])')].map((a) => {
    const c = document.createElement('canvas');
    c.className = 'tile-3d';
    c.width = c.height = 96;
    c.setAttribute('aria-hidden', 'true');
    return { a, c, ctx: c.getContext('2d'), m: null, angle: 0, hover: false };
  });
  const paint = (t) => {
    draw(small, t.m, t.angle);
    t.ctx.clearRect(0, 0, 96, 96);
    t.ctx.drawImage(small.domElement, 0, 0);
  };
  tiles.forEach((t) => {
    t.a.addEventListener('pointerenter', () => { t.hover = true; });
    t.a.addEventListener('pointerleave', () => { t.hover = false; });
    load(t.a.dataset.model).then((m) => {
      t.m = m;
      t.a.querySelector('.plate-model')?.after(t.c);
      t.a.classList.add('live');
      paint(t);
    }).catch(() => {});
  });

  let last = 0;
  const tick = (now) => {
    const dt = last ? Math.min(now - last, 100) : 0;
    last = now;
    if (seen && !document.hidden && !calm()) {
      if (on && drag === null) { angle -= (dt * 2 * Math.PI) / 12000; draw(big, on, angle); }
      for (const t of tiles) {
        if (t.m && (t.hover || t.a.getAttribute('aria-current') === 'true')) { t.angle -= (dt * 2 * Math.PI) / 8000; paint(t); }
      }
    }
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
}
