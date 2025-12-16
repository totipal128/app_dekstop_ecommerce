<script>
  import { input } from "flowbite-svelte";

  let {
    data = [
      [
        { is_input: true, type: "text", value: "adasd" },
        { is_input: true, type: "text", value: "adasd" },
        { is_input: true, type: "number", value: 0 },
        { is_input: true, type: "number", value: 5000 },
        { is_input: false, type: "number", value: 0 },
      ],
      // { code:0, satuan: 0, harga: 0, total: 0 }
    ],
    headers = ["Code", "Nama Barang", "Satuan", "Harga", "Total"],
  } = $props();

  let data_c = $state(structuredClone(data));
  let index_row = $state(0);

  function parseNumber(v) {
    if (!v) return 0;
    return Number(v.toString().replace(/\./g, ""));
  }

  function onInput(rowIndex, key, event) {
    const value = event.target.value;
    data_c[rowIndex][key] = value;

    // Recalculate total
    const satuan = parseNumber(data_c[rowIndex].satuan);
    const harga = parseNumber(data_c[rowIndex].harga);
    data_c[rowIndex].total = satuan * harga;
  }

  $effect(() => {
    if (data_c.length - 1 === index_row) {
      data_c.push([
        { is_input: true, type: "text", value: "" },
        { is_input: true, type: "text", value: "" },
        { is_input: true, type: "number", value: 0 },
        { is_input: true, type: "number", value: 0 },
        { is_input: false, type: "number", value: 0 },
      ]);
    }

    for (const row of data_c) {
      const totalIndex = row.length - 1;
      const qtyIndex = row.length - 3;
      const priceIndex = row.length - 2;
      console.log(totalIndex, qtyIndex, priceIndex);

      row[totalIndex].value =
        parseNumber(row[qtyIndex].value) * parseNumber(row[priceIndex].value);
    }
  });
</script>

<div>
  <div class="search-wrapper">
    <input type="text" class="search" placeholder="Search..." />
  </div>

  <div class="table-wrapper">
    <table class="table-container">
      <thead>
        <tr>
          {#each headers as header}
            <th>{header}</th>
          {/each}
        </tr>
      </thead>
      <tbody class="body-table-container">
        {#each data_c as dataRow, i (i)}
          <tr>
            {#each dataRow as coll, ci (ci)}
              <td>
                {#if coll.is_input}
                  <input
                    type={coll.type}
                    class="input-table"
                    bind:value={coll.value}
                    on:input={() => (index_row = i)}
                    placeholder="input..."
                  />
                {:else}
                  <input
                    type={coll.type}
                    class="input-table"
                    bind:value={coll.value}
                    placeholder="input..."
                    readonly
                  />
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .input-table {
    width: 100%;
    padding: 6px;
    box-sizing: border-box;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  div.search-wrapper {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 10px;
  }

  .search {
    box-shadow: 0 2px 8px rgba(93, 87, 87, 0.2);
    padding: 8px;
    width: 200px;
    border: 1px solid #ccc;
    border-radius: 15px;
  }

  .search:focus {
    outline: none;
    border: 2px solid #86beff;
    box-shadow: 0 0 5px rgba(136, 136, 136, 0.5);
  }

  .table-wrapper {
    width: 100%;
    max-height: 20%;
    overflow-x: auto;
    overflow-y: auto;
    background-color: #ffffff;
    padding: 10px;
    padding-top: 0px;
    border-radius: 5px;
  }

  .table-container {
    width: 100%;
    max-height: 50%;
    margin: 20px 0;
    border-collapse: separate;
    border-spacing: 0;
  }

  .table-container thead th {
    position: sticky;
    top: 0;
    z-index: 10; /* WAJIB */
    background: #cfe6ff; /* WAJIB (biar tidak tembus) */
    padding: 12px;
  }
  .table-container thead {
    background-color: #bde0fe;
  }

  .table-container th,
  td {
    border-bottom: 1px solid #f8f9fa;
    padding: 8px;
    text-align: left;
  }

  /* th {
    background-color: #bde0fe;
  } */

  .table-container tr:nth-child(even) {
    background-color: #fafafa;
  }

  .table-container tr:hover {
    background-color: #ced4da;
  }
</style>
