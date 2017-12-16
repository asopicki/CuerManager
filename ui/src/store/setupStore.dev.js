import { createStore, applyMiddleware } from 'redux';

import api from '../middleware/api'
import rootReducer from '../reducers';

const setupStore = preloadedState => {
	const store = createStore(
		rootReducer,
		Object.assign({}, preloadedState, {cuesheetSearch: { serverPort: 8087}}),
		applyMiddleware(api)
	)

	if (module.hot) {
		// Enable Webpack hot module replacement for reducers
            module.hot.accept('../reducers', () => {
              store.replaceReducer(rootReducer)
        })
	}

	return store
}

export default setupStore