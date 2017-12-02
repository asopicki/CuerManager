import { createStore, applyMiddleware, compose } from 'redux';

import api from '../middleware/api'
import rootReducer from '../reducers';

const setupStore = preloadedState => createStore(
	rootReducer,
	preloadedState,
	() => applyMiddleware(api)
)

export default setupStore