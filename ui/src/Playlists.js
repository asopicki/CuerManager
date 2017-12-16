import React, { Component } from 'react';
import { connect } from 'react-redux';
import {playlistSearch} from './actions/playlists';
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

		const cuesheetList = playlist.cuesheets.map(cuesheet => {
			let url = "/cuesheets/"+cuesheet.id;
			return (<li><a href={url} target="_blank">{cuesheet.title}</a></li>);
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
	}
}

const mapStateToPlaylistProps = state => {
	return {
		playlists: state.playlists.playlistsResult
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
        <tr>
           <td><Link to={url} >{props.name}</Link></td>
           <td className="textcenter">n/a</td>
           <td className="textcenter">TODO</td>
        </tr>
    );

};

function PlaylistResult(props) {
    const listRows = props.searchResult.map(result => {
        return (<PlaylistRow playlistId={result.id} name={result.name} key={result.id} />)
    });

    return (
        <div className="resultList">
            <h2>Playlists</h2>
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
    );
};

class PlaylistContainer extends Component {

    fetchPlaylists() {
        /*let self = this
        let request = new Request('/playlists');

        fetch(request).then(function (response) {
            return response.json();
        }).then((result) => {
            self.setState({
                playlistResult: _.orderBy(result, ['name'], ['asc']),
                refresh: false
            })
        });*/
        //TODO: Dispatch action FETCH_PLAYLIST
        this.props.fetchPlaylists();
    }

	render() {
		if (this.props.refresh) {
			this.fetchPlaylists();
		}

		return (
                        <div>
                            <PlaylistResult searchResult={this.props.searchResult} />
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
		refresh: state.playlists.refresh
	}
}

const mapDispatchToProps = dispatch => {
	return {
		fetchPlaylists: () => {
        			dispatch(playlistSearch())
        }
	}
}

const PlaylistSearch = connect(mapStateToProps, mapDispatchToProps)(PlaylistContainer)

export default PlaylistManager;