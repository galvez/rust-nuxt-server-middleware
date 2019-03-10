<template>
  <p>{{ neonMessage }}</p>
</template>

<script>
export default {
  asyncData({ res }) {
    let neonMessage
    if (process.server) {
      neonMessage = res.neonMessage
    }
    return { neonMessage }
  }
}
</script>

