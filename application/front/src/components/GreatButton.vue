<template>
    <button @click="onClick()" class="btn"> {{ text }} </button>
</template>

<script lang="ts">
export default {
    name: 'GreatButton',
    props: {
        text: String,
    },
    methods: {
        onClick() {
            this.$emit('btn-click')
        },
    },
}
</script>

<style scoped>
.btn{
    box-shadow:inset 0 0 0 99999px rgba(0,0,0,0.2);
    font-size: 18px;
}
.btn:hover{
    box-shadow:none;
}
</style>