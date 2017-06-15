<template>
  <tr>
    <td>
      <a :href="'http://localhost:8087/cuesheets/' + cuesheet.id" target="_blank" :title="cuesheet.title" rel="noopener noreferer">
        {{cuesheet.title}}
      </a>
    </td>
    <td>
      {{cuesheet.rhythm}}
    </td>
    <td>
      {{cuesheet.phase}} <span v-if="cuesheet.plusfigures !== 'null'">{{cuesheet.plusfigures}}</span>
    </td>
    <td v-if="cuesheet.plusfigures !== 'null'">{{cuesheet.plusfigures}}</td>
    <td v-else></td>
  </tr>
</template>

<script>
export default {
  name: 'cuesheet-item',
  props: ['cuesheet']
}
</script>
