export default function two_crystal_balls(breaks: boolean[]): number {
  const rootN = Math.floor(Math.sqrt(breaks.length));
  let safeIndex = 0;
  let result = -1;

  for (let i = safeIndex; i < breaks.length; i += rootN) {
    if (breaks[i]) {
      break;
    }

    safeIndex = i;
  }

  for (let i = safeIndex; i <= safeIndex + rootN; i++) {
    if (breaks[i]) {
      result = i;
      break;
    }
  }

  return result;
}
