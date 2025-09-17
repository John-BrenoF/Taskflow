import { invoke } from "@tauri-apps/api/tauri";

let myChart;

async function refreshTasks() {
  const tasks = await invoke('list_tasks');
  renderTasks(tasks);
  drawChart(tasks);
}

function renderTasks(tasks) {
  const container = document.getElementById('tasks');
  container.innerHTML = '';
  tasks.forEach(t => {
    const el = document.createElement('div');
    el.className = 'task';
    el.innerHTML = `
      <b>${t.title}</b> <small>${t.category ?? ''}</small>
      <p>${t.description ?? ''}</p>
      <div>
        <button data-id="${t.id}" data-action="toggle" data-status="Todo">Todo</button>
        <button data-id="${t.id}" data-action="toggle" data-status="InProgress">Em andamento</button>
        <button data-id="${t.id}" data-action="toggle" data-status="Done">Concluído</button>
        <button data-id="${t.id}" data-action="delete" class="delete">Remover</button>
      </div>
    `;
    container.appendChild(el);
  });
}

function drawChart(tasks) {
}

document.addEventListener("DOMContentLoaded", () => {
  const tasksContainer = document.getElementById("tasks");

  document.getElementById('add').addEventListener('click', async () => {
    const title = document.getElementById('title').value;
    const desc = document.getElementById('desc').value || null;
    const due = document.getElementById('due').value || null;
    const cat = document.getElementById('category').value || null;
    const rec = document.getElementById('recurrence').value || null;

    if (!title) return alert('Título é obrigatório!');

    await invoke('create_task', {
      title,
      description: desc,
      due_date: due,
      category: cat,
      recurrence: rec
    });

    document.getElementById('title').value = '';
    document.getElementById('desc').value = '';
    document.getElementById('due').value = '';
    document.getElementById('category').value = '';
    document.getElementById('recurrence').value = 'once';

    refreshTasks();
  });

  tasksContainer.addEventListener("click", async (e) => {
    const target = e.target;
    const action = target.dataset.action;
    const id = target.dataset.id;

    if (action === "delete") {
      await invoke('delete_task', { id });
      refreshTasks();
    } else if (action === "toggle") {
      const status = target.dataset.status;
      await invoke('toggle_status', { id, status });
      refreshTasks();
    }
  });

  refreshTasks();
});
