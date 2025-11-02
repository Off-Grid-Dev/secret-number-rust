import init, {
  pick_new_number as pickNewNumber,
  check_number as checkNumber,
} from "./pkg/secret_number.js";

async function run() {
  // Initialize wasm
  await init();
  // Grab html elements
  const userLevel = document.getElementById("level");
  const attempts = document.getElementById("attempts");
  const totalAttempts = document.getElementById("totalAttempts");
  const guessInput = document.getElementById("guessInput");
  const response = document.getElementById("response");
  const secretDebug = document.getElementById("response");
}
