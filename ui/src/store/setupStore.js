if (process.env.NODE_ENV === 'production') {
  module.exports = require('./setupStore.prod')
} else {
  module.exports = require('./setupStore.dev')
}