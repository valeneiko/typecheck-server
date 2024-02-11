async function funcAsync() {
  return Promise.resolve();
}

function func() {
  return 7;
}

export async function test() {
  func();
  funcAsync();
}
