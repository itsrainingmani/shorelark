import * as sim from 'lib-simulation-wasm';

const simulation = new sim.Simulation();
const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext('2d');

ctxt.scale(viewportScale, viewportScale);

ctxt.fillStyle = 'rgb(0, 0, 0)';

for (const animal of simulation.world().animals) {
	ctxt.fillRect(animal.x * viewportWidth, animal.y * viewportHeight, 15, 15);
}
