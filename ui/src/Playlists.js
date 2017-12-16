import React, { Component } from 'react';
import { connect } from 'react-redux';
import {playlistSearch, createPlaylist, createPlaylistName } from './actions/playlists';
import {
  Route,
  Link,
  withRouter
} from 'react-router-dom'
//import _ from 'lodash';

class Playlist extends Component {

	findPlaylist(id) {
        return this.props.playlists.find(element => element.id === id);
	}

	render() {
		const playlist = this.findPlaylist(this.props.match.params.id)

		if (playlist) {

			const cuesheetList = playlist.cuesheets.map(cuesheet => {
				let url = "http://localhost:"+this.props.serverPort+"/cuesheets/"+cuesheet.id;
				return (<li key={cuesheet.id}><a href={url} target="_blank">{cuesheet.title}</a></li>);
			});

			return (
				<div>
					<h2>Playlist - {playlist.name}</h2>

					<h3>Cuesheet list</h3>

					<ul>
						{cuesheetList}
					</ul>
				</div>
			);
		} else {
			return (
				<div>
					<h2>Playlist not found!</h2>
				</div>
			)
		}
	}
}

const mapStateToPlaylistProps = state => {
	return {
		playlists: state.playlists.playlistsResult,
		serverPort: state.cuesheetSearch.serverPort
	}
}

const mapDispatchToPlaylistProps = dispatch => {
	return {

	}
}

const PlaylistWithRouter = withRouter(connect(mapStateToPlaylistProps, mapDispatchToPlaylistProps)(Playlist));


function PlaylistRow(props) {

    let url = '/playlists/' + props.playlistId;

    return  (
        <tr key={props.playlistId}>
           <td><Link to={url} >{props.name}</Link></td>
           <td className="textcenter">n/a</td>
           <td className="textcenter">TODO</td>
        </tr>
    );

};

class PlaylistContainer extends Component {

	constructor(props) {
        super(props)

        this.createPlaylist = this.createPlaylist.bind(this);
        this.createPlaylistName = this.createPlaylistName.bind(this);
        this.fetchPlaylists = this.fetchPlaylists.bind(this);
    }

	createPlaylistName(event) {
		this.props.createPlaylistName(event.target.value);
	}

    createPlaylist() {
        this.props.createPlaylist(this.props.playlistName);
        document.getElementById("playlist").value = '';
    }


    fetchPlaylists() {
        this.props.fetchPlaylists();
    }

	render() {
		if (this.props.refresh) {
			console.log("Refreshing:", this.props.refresh);
			this.fetchPlaylists();
		}

		const listRows = this.props.searchResult.map((result, index) => {
                return (<PlaylistRow key={index} playlistId={result.id} name={result.name} />)
        });

		return (
                <div>
                    <h2>Playlists</h2>
                    <div>
                        <label htmlFor="playlist">Name:</label>
                        <input type="text" name="playlist" id="playlist" placeholder="Name of your playlist"
                            onChange={this.createPlaylistName}/>
                        <button onClick={this.createPlaylist}>Create playlist</button>
                        <button onClick={this.fetchPlaylists}>Refresh playlists</button>
                    </div>
                    <p></p>
                    <div className="resultList">
                        <table className="textleft">
                            <thead>
                                <tr>
                                    <td className="textcenter">Name</td>
                                    <td className="textcenter">Event</td>
                                    <td className="textcenter">Actions</td>
                                </tr>
                            </thead>
                            <tbody>
                                {listRows}
                            </tbody>
                        </table>
                    </div>
                </div>
        );
	}

}

function PlaylistManager(props) {

    return (
        <div>
            <Route exact path="/playlists" component={PlaylistSearch} />
            <Route strict path="/playlists/:id" component={PlaylistWithRouter} />
        </div>
    );
}


const mapStateToProps = state => {
	return {
		searchResult: state.playlists.playlistsResult,
		refresh: state.playlists.refresh,
		playlistName: state.playlists.createForm.name
	}
}

const mapDispatchToProps = dispatch => {
	return {
		fetchPlaylists: () => {
        	dispatch(playlistSearch())
        },
        createPlaylist: (name) => {
            dispatch(createPlaylist(name))
        },
        createPlaylistName: (name) => {
            dispatch(createPlaylistName(name))
        }
	}
}

const PlaylistSearch = connect(mapStateToProps, mapDispatchToProps)(PlaylistContainer)

export default PlaylistManager;