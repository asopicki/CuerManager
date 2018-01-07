import { createStore, applyMiddleware, compose } from 'redux';

import api from '../middleware/api'
import rootReducer from '../reducers';

const setupStore = preloadedState => createStore(
	rootReducer,
    Object.assign({}, preloadedState, {cuesheetSearch: { serverPort: 8000}}),
    applyMiddleware(api)
)

export default setupStore