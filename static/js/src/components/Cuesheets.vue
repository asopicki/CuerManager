<template>
  <div class="cuesheets">
    <div v-for="phase in phases">
      <h2>Phase {{phase}}</h2>
      <table>
        <thead>
          <tr>
            <td>Name</td>
            <td>Rhythm</td>
            <td>Phase</td>
            <td>Plusfigure</td>
          </tr>
        </thead>
        <tbody>
          <CuesheetItem v-for="item in byPhase(cuesheets, phase) " v-bind:cuesheet="item" :key="item.id"></CuesheetItem>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style>
  table {
    width: 100%;
    border: 1px solid black;
    border-collapse: collapse;
  }

  thead td {
    font-weight: bold;
    border: 1px solid black;
    border-collapse: collapse;
    background-color: rgb(38, 73, 114);
    color: white;
  }

  tbody td {
    border: 1px solid black;
    border-collapse: collapse;
  }

  tr:nth-child(even) {
    background-color: rgba(117, 122, 127, 0.7);
  }
  tr:hover {
    background-color: rgba(68, 128, 196, 0.9);
  }

  td a {
    color: rgb(25, 20, 91);
    font-weight: bold;
  }
</style>

<script>
import CuesheetItem from '@/components/CuesheetItem'
import _ from 'lodash'

let _phases = ['II', 'III', 'IV', 'V', 'VI']

export default {
  data: function () {
    let request = new Request('/search/all')
    let data = {
      cuesheets: [],
      phases: _phases
    }

    fetch(request).then(function (response) {
      return response.json()
    }).then(function (response) {
      data.cuesheets = _.orderBy(response, ['title'], ['asc'])
    })

    return data
  },
  computed: {
    _ () {
      return _
    }
  },
  methods: {
    byPhase: function (cuesheetList, phase) {
      if (cuesheetList != null) {
        return _.filter(cuesheetList, function (cuesheet) {
          return cuesheet.phase === phase
        })
      }

      return cuesheetList
    }
  },
  components: {
    CuesheetItem
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
