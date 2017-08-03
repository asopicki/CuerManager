<template>
  <div class="cuesheets">
    <div>
      <input type="text" name="query" v-model="query" placeholder="Enter search query" size="30" />
      <button v-on:click="search()">Search</button>
      <p>
        <button v-for="phase in phases" v-on:click="searchByPhase(phase)">Phase {{phase}}</button>
      </p>
      <p>
        <button v-for="rhythm in rhythms" v-on:click="searchByRhythm(rhythm)">{{rhythm}}</button>
      </p>
    </div>

    <div>
      <h2>Search result</h2>
      <table>
        <thead>
          <tr>
            <td>Name</td>
            <td>Rhythm</td>
            <td>Phase</td>
          </tr>
        </thead>
        <tbody>
          <CuesheetItem v-for="item in cuesheets" v-bind:cuesheet="item" :key="item.id"></CuesheetItem>
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
    let data = {
      cuesheets: [],
      phases: _phases,
      rhythms: [
        'Cha-Cha-Cha', 'Rumba', 'Two Step', 'Foxtrot', 'Bolero', 'Waltz', 'Tango', 'Quickstep',
        'Samba', 'Single Swing', 'Slow Two Step', 'West Coast Swing', 'Jive', 'Mambo', 'Paso Doble',
        'Hesitation Canter Waltz'
      ],
      query: ''
    }

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
    },

    searchByPhase: function (phase) {
      this.query = 'phase:' + phase

      this.search()
    },
    searchByRhythm: function (rhythm) {
      this.query = rhythm

      this.search()
    },

    search: function () {
      let request = new Request('/search/' + this.query)

      if (this.query.includes('phase:')) {
        request = new Request('/search/phase/' + this.query.substr(6))
      }

      var self = this

      fetch(request).then(function (response) {
        return response.json()
      }).then(function (response) {
        self.cuesheets = _.orderBy(response, ['title'], ['asc'])
      })
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
