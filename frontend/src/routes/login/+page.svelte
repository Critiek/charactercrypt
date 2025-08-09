<script lang="ts">
    async function get_code() {
        const response = await fetch("http://localhost:3000/code");
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }
        console.log("Code Generated");
    }

    async function submit_code() {
        const response = await fetch("http://localhost:3000/submit_code", {
            method: "POST",
        });
        console.log(response)
    }
</script>

<button onclick={get_code} class="text-6xl border-2">Get Code</button>
<input type="text" />
<button onclick={submit_code} class="text-6xl border-2">Submit Code</button>
