import Vue from 'vue'
import Router from 'vue-router'
import Cuesheets from '@/components/Cuesheets'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'cuesheets',
      component: Cuesheets
    }
  ]
})
