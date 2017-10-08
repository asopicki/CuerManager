import React, { Component } from 'react';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'
import Search from './Search.js';
import PlaylistManager from './Playlists.js';
import './App.css';



class App extends Component {

	render() {
		 return (<Router>
		    <div>
		        <div class="menu">
		            <ul>
		                <li><Link to="/">Search</Link></li>
		                <li><Link to="/playlists">Playlists</Link></li>
	                </ul>
		        </div>

				<div className="App">
					<h1>Cueing Manager - Cuesheet search</h1>
		            <Route exact path="/" component={Search} />
		            <Route path="/playlists" component={PlaylistManager} />
		        </div>
		    </div>
		 </Router>);
	}
}

export default App;