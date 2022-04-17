export function render(data) {
  let container = document.getElementById("minesweeper");
  container.innerHTML = "";

  let y = 0;
  let board = data.split("\n").map((row) => row.trim().split(/\s+/));

  for (let row of board) {
    let x = 0;
    let rowContainer = document.createElement("div");
    rowContainer.classList.add("row");

    for (let field of row) {
      let [i, j] = [x, y];
      let fieldContainer = document.createElement("a");
      fieldContainer.innerText = field;
      fieldContainer.classList.add("field");
      fieldContainer.href = "#";

      fieldContainer.addEventListener("click", (evt) => {
        evt.preventDefault();
        Minesweeper.open(i, j);
      });

      fieldContainer.addEventListener("contextmenu", (evt) => {
        evt.preventDefault();
        Minesweeper.toggleFlag(i, j);
      });

      rowContainer.appendChild(fieldContainer);
      x++;
    }

    container.appendChild(rowContainer);
    y++;
  }
}
