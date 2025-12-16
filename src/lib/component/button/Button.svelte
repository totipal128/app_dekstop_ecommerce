<script>
  const { text = "Button", type = "primary", ...rest } = $$props;
</script>

<button {...rest}>{text}</button>

<style>
  button {
    padding: 10px 20px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
    color: white;
    background-color: var(--button-bg, #007bff);
  }

  button.clicked {
    transform: scale(0.98);
  }

  button[type="primary"] {
    --button-bg: #007bff;
  }

  button[type="secondary"] {
    --button-bg: #6c757d;
  }

  button[type="success"] {
    --button-bg: #28a745;
  }

  button[type="danger"] {
    --button-bg: #dc3545;
  }
</style>
