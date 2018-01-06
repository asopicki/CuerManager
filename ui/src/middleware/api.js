import * as constants from '../constants/ActionTypes';


export default store => next => action => {
	switch(action.type) {
	    case constants.API: {
			console.log('Got an API request from', action.meta.origin, "to", action.payload.url, "(", action.payload.method, ")");
            let request = new Request(action.payload.url, {
                method: action.payload.method,
                body: action.payload.body,
                headers: action.payload.headers
            });
            const responseAction= data => {
                const finalAction = Object.assign({}, action, data)
                return finalAction
            }

			switch(action.payload.method) {
				case 'GET': {
					return fetch(request).then(
                        response => response.json().then(data => {
                            return next(responseAction({
                                payload: {
                                    result: data,
                                },
                                type: action.payload.success
                            }))
                        }, () => {
                            let resultAction = responseAction({
                                error: true
                            });
                            action.payload.error = new Error('GET request failed!');
                            next(resultAction);
                        }),
                        error => {
                            let resultAction = responseAction({
                                payload: Object.assign({}, action.payload, {error: error}),
                                error: true
                            });

                            next(resultAction)
                        }
                    );
				}
				case 'PUT':
				case 'DELETE': {
                    request.headers.append('Content-Type', 'application/json');
                    return fetch(request).then(
                        response => response.json().then(data => {
                            return next(responseAction({
                                payload: {
                                    result: data,
                                },
                                type: action.payload.success
                            }))
                        }, () => {
                            let resultAction = responseAction({
                                error: true
                            });
                            action.payload.error = new Error('PUT request failed!');
                            next(resultAction);
                        }),
                        error => {
                            let resultAction = responseAction({
                                payload: Object.assign({}, action.payload, {error: error}),
                                error: true
                            });

                            next(resultAction)
                        }
                    );
                }
				default: {
				}
			}
			break;
	    }
		default: {
			return next(action)
		}
	}
}