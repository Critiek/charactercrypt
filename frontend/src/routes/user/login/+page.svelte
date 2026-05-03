<script lang="ts">
    let magic_code = "";

    async function get_code() {
        const response = await fetch("http://localhost:3000/api/gen_auth_code");
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }
        console.log("Code Generated");
    }

    async function submit_code() {
        const response = await fetch("http://localhost:3000/api/submit_code", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                user: "test-user",
                magic_code,
            }),
        });
        console.log(response);
    }
</script>

<form>
    <button onclick={get_code} class="text-6xl border-2">Get Code</button>
    <button onclick={submit_code} class="text-6xl border-2">Submit Code</button>
    <input type="text" bind:value={magic_code} class="text-6xl border-2" />
</form>