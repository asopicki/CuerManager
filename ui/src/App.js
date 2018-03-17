import React, { Component } from 'react';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'
import Search from './Search.js';
import PlaylistManager from './Playlists.js';
import './App.css';
import Modal from 'react-modal';

Modal.setAppElement('#root')

class App extends Component {

	render() {
		 return (<Router>
		    <div className="container">
		        <div className="header">
                    <h1>Cueing Manager</h1>
                    <nav className="topNavigation">
                        <ul>
                            <li><Link to="/">Search</Link></li>
                            <li><Link to="/playlists">Playlists</Link></li>
                        </ul>
                    </nav>
                </div>
				<div className="App">
		            <Route exact path="/" component={Search} />
		            <Route path="/playlists" component={PlaylistManager} />
		        </div>
		        <div className="footer">
		            <em>&copy; 2017 - Alexander Sopicki</em>
		        </div>
		    </div>
		 </Router>);
	}
}

export default App;