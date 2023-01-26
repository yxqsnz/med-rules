import { generate, default as init } from "./pkg/w_med.js";
const render = document.querySelector("#render");

const updateStatusText = (text) => {
  document.querySelector("#status").textContent = text;
};

const renderPeople = (pep) => {
  const root = document.createElement("div");
  const p = document.createElement("p");
  p.textContent = `${pep.gender} ${pep.eye_color} ${pep.skin_color}`;

  root.appendChild(p);
  return root;
};

const generateClick = () => {
  const rawJSON = generate(4, 8);
  render.textContent = "";
  if (!rawJSON) {
    alert(
      "deu erro ao gerar o JSON chefia, mas nn sei o erro, pergunte ao dev p ver se ele consegue resolver"
    );
  }

  console.log(rawJSON);
  const json = JSON.parse(rawJSON);

  updateStatusText(`Total de descedentes: ${json.total_generations}`);

  const families = json.simulator.families;

  const root = document.createElement("ul");
  render.appendChild(root);

  for (const family of families) {
    const li = document.createElement("li");
    const parent = document.createElement("div");
    parent.className = "flx-row";

    const mother = renderPeople(family.mother);
    const father = renderPeople(family.father);

    father.className = "ml";
    parent.appendChild(mother);
    parent.appendChild(father);

    li.appendChild(parent);

    const childrenRoot = document.createElement("ul");

    for (const child of family.children) {
      const childElement = document.createElement("li");

      childElement.appendChild(renderPeople(child));
      childrenRoot.appendChild(childElement);
    }

    li.appendChild(childrenRoot);

    root.appendChild(li);
  }
};

async function run() {
  await init();
  const btn = document.querySelector("#generate-btn");
  btn.addEventListener("click", generateClick);
}

run();
