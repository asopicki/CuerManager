import React, { Component } from 'react';
import { connect } from 'react-redux';
import {playlistSearch, createPlaylist, createPlaylistName, removePlaylist, removeCuesheet } from './actions/playlists';
import {
  Route,
  Link,
  withRouter
} from 'react-router-dom'

class CuesheetRow extends Component {

	constructor(props) {
		super(props);

		this.removeCuesheet = this.removeCuesheet.bind(this);
	}

	removeCuesheet() {
		this.props.removeHandler(this.props.cuesheet.id);
	}

	render() {
		let cuesheet = this.props.cuesheet;

		return (
			<tr>
	            <td><a href={this.props.url} target="_blank">{cuesheet.title}</a></td>
	            <td className="textcenter" onClick={this.removeCuesheet}>Remove</td>
	        </tr>
		);
	}
}

class Playlist extends Component {
	playlist: null

	constructor(props) {
		super(props);

		this.removeCuesheet = this.removeCuesheet.bind(this);
	}

	findPlaylist(id) {
        return this.props.playlists.find(element => element.uuid === id);
	}

	removeCuesheet(cuesheet_id) {
		this.playlist.cuecards = this.playlist.cuecards.filter((element) => {
			return element.id !== cuesheet_id;
		})

		return this.props.removeCuesheet(this.playlist.id, cuesheet_id)
	}

	render() {
		this.playlist = this.findPlaylist(this.props.match.params.id)

		if (this.playlist) {

			const cuesheetList = this.playlist.cuecards.map(cuesheet => {
				let url = "http://localhost:"+this.props.serverPort+"/cuesheets/"+cuesheet.id;
				return (
					<CuesheetRow key={cuesheet.id} cuesheet={cuesheet} removeHandler={this.removeCuesheet}
						url={url}/>
				);
			});

			return (
				<div className="resultList">
					<h2>Playlist - {this.playlist.name}</h2>

					<h3>Cuesheet list</h3>

					<table className="textleft">
						<thead>
							<tr>
								<td>Name</td>
								<td>Actions</td>
							</tr>
						</thead>
						<tbody>
							{cuesheetList}
						</tbody>
					</table>
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
		removeCuesheet: (id, cuesheet_id) => {
            dispatch(removeCuesheet(id, cuesheet_id))
        }
	}
}

const PlaylistWithRouter = withRouter(connect(mapStateToPlaylistProps, mapDispatchToPlaylistProps)(Playlist));


class PlaylistRow extends Component {

	constructor(props) {
		super(props)

		this.removePlaylist = this.removePlaylist.bind(this);
	}

	removePlaylist() {
		this.props.removeHandler(this.props.playlistId)
	}

	render() {

	    let url = '/playlists/' + this.props.playlistId;

	    return  (
	        <tr key={this.props.playlistId}>
	           <td><Link to={url} >{this.props.name}</Link></td>
	           <td className="textcenter">n/a</td>
	           <td className="textcenter" onClick={this.removePlaylist}>remove</td>
	        </tr>
	    );

	}
}

class PlaylistContainer extends Component {

	constructor(props) {
        super(props)

        this.createPlaylist = this.createPlaylist.bind(this);
        this.createPlaylistName = this.createPlaylistName.bind(this);
        this.fetchPlaylists = this.fetchPlaylists.bind(this);
        this.removePlaylist = this.removePlaylist.bind(this);
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

    removePlaylist(id) {
        this.props.removePlaylist(id);
    }

	render() {
		if (this.props.refresh) {
			this.fetchPlaylists();
		}

		const listRows = this.props.searchResult.map((result, index) => {
                return (<PlaylistRow key={result.id} playlistId={result.id} uuid={result.uuid}  name={result.name}
                    removeHandler={this.removePlaylist}/>)
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
        },
        removePlaylist: (id) => {
            dispatch(removePlaylist(id))
        }
	}
}

const PlaylistSearch = connect(mapStateToProps, mapDispatchToProps)(PlaylistContainer)

export default PlaylistManager;