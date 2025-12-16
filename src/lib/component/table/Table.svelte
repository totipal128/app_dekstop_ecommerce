<script>
  let {
    data = [
      ["Data 1", "Data 2", "Data 3"],
      ["Data 4", "Data 5", "Data 6"],
    ],
    headers = ["Header 1", "Header 2", "Header 3"],
    is_action = false,
    onEdit = () => {},
    onHapus = () => {},
    onDetail = () => {},
  } = $props();

  function getId(id) {
    console.log(id);
  }
</script>

<div class="table-wrapper">
  <table class="table-container">
    <thead>
      <tr>
        {#each headers as header}
          <th>{header}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each data as dataRow}
        <tr>
          {#each dataRow as cell}
            <td>{cell}</td>
          {/each}
          {#if is_action}
            <td class="action">
              <button onclick={() => onDetail(dataRow[0])} class="view"
                >Detail</button
              >
              <button onclick={() => onEdit(dataRow[0])} class="edit"
                >Edit</button
              >
              <button class="delete">Delete</button>
            </td>
          {/if}
        </tr>
      {/each}
    </tbody>
  </table>
  <div class="pagination-wrapper">
    <button> prev </button>
    <button> 1 </button>
    <button> 2 </button>
    <button> 3 </button>
    <button> 4 </button>
    <button> next </button>
  </div>
</div>

<style>
  .action button {
    margin-right: 5px;
    padding: 5px 10px;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }

  .action .view {
    background-color: #306be1;
    border-radius: 10px;
    color: white;
  }

  .action .view:hover {
    background-color: #8695d0;
  }

  .action .edit {
    background-color: #4caf50;
    border-radius: 10px;
    color: white;
  }

  .action .delete {
    background-color: #f44336;
    border-radius: 10px;
    color: white;
  }

  .action .edit:hover {
    background-color: #45a049;
  }
  .action .delete:hover {
    background-color: #da190b;
  }

  .pagination-wrapper {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 10px;
    gap: 5px;
  }
  .pagination-wrapper button {
    background-color: #dee2e6;
    border: none;
    color: white;
    padding: 10px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin: 4px 2px;
    cursor: pointer;
    border-radius: 5px;
  }
  .pagination-wrapper button:hover {
    background-color: #adb5bd;
    transform: scale(1, 1.23);
  }

  .table-wrapper {
    width: 100%;
    overflow-x: auto;
    background-color: #ffffff;
    padding: 10px;
    border-radius: 5px;
  }
  .table-container {
    margin: 20px 0;
    font-family: Arial, sans-serif;
    overflow-x: auto;
    overflow: hidden;
    border-radius: 5px;
    background-color: #f2f2f2;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }
  thead {
    background-color: #bde0fe;
  }

  th,
  td {
    border-bottom: 1px solid #f8f9fa;
    padding: 8px;
    text-align: left;
  }

  /* th {
    background-color: #bde0fe;
  } */
  tr:nth-child(even) {
    background-color: #fafafa;
  }
  tr:hover {
    background-color: #ced4da;
  }
</style>
