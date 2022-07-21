let count = 0;

function render() {
  const app = document.getElementById("app");
  app.innerHTML = "";

  const button = document.createElement("button");
  button.addEventListener("click", handleClick);
  button.innerText = "Count + 1";

  const h1 = document.createElement("h1");
  h1.innerText = `Count: ${count}`;

  app.appendChild(h1);
  app.appendChild(button);
}

render();

function handleClick() {
  count = count + 1;
  render();
}

