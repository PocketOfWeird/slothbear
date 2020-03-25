const { exec } = require('child_process');
const uuidV4 = require('uuid/v4');

const settings = (renderId, startF, endF, proj, scene, outDir, format, cam, step=1, pad=4, resX=1920, resY=1080) => ({
  active: false,
  pending: true,
  done: false,
  canceled: false,
  error: false,
  errInfo: '',
  renderId,
  startF,
  endF,
  proj,
  scene,
  outDir,
  format,
  cam,
  step,
  pad,
  resX,
  resY
});

const splitIntoChunks = (settings, chunkSize) => {
  let chunks = [];
  for (let frame = settings.startF; frame <= settings.endF; frame+=chunkSize) {
    let chunk = {
      ...settings,
      chunkId: uuidV4(),
      startF: frame,
      endF: Math.min(frame + chunkSize - 1, settings.endF)
    }
    chunks.push(chunk);
  }
  return chunks;
}

const startChunk = (chunk, cb) => {
    const command = chunk => `"${process.env.MAYA_BIN_DIR}\\Render.exe" -r file -s ${chunk.startF} -e ${chunk.endF} -b ${chunk.step} -proj "${chunk.proj}" -rd "${chunk.outDir}" -fnc name.#.ext -pad ${chunk.pad} -of ${chunk.format} -cam "${chunk.cam}" -x ${chunk.resX} -y ${chunk.resY} -verb "${chunk.scene}"`;
    const child = exec(command(chunk));
    child.stdout.on('data', info => {
      cb(false, false, info);
    });
    child.stderr.on('data', errorInfo => {
      cb(false, errorInfo, false);
    });
    child.on('error', error => {
      cb(error, false, false);
    });
    child.on('exit', (code, signal) => {
      cb(false, signal, code, false);
    });
}

module.exports = {
  settings,
  splitIntoChunks,
  startChunk
};
