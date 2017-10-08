import React, { Component } from 'react';
import {
  Route,
  Link,
  withRouter
} from 'react-router-dom'
import _ from 'lodash';

var playlistData = [{
  id: "1234566",
  name: "Bavarian Festival",
  Cuesheets: [
      {
          title: "All shook up",
          id: "AV6t1klvHh_QOGz32LnF"
      },
      {
          title: "Rock-Paper-Scissors",
          id: "AV6t1kl3Hh_QOGz32LnI"
      },
      {
          title: "Watermark",
          id: "AV6t1kkEHh_QOGz32Lmi"
      }
  ]
},
{
  id: "abcdefg",
  name: "Tamara Clubabend",
  Cuesheets: [
   {
          title: "Test",
          id: "AV6t1klvHh_QOGz32LnF"
      },
      {
          title: "Test",
          id: "AV6t1kl3Hh_QOGz32LnI"
      },
      {
          title: "Watermark",
          id: "AV6t1kkEHh_QOGz32Lmi"
      }
  ]

}];

class Playlist extends Component {

	constructor() {
		super();

		this.state = {
			playlist: {
				name: "",
				Cuesheets: [],
				id: ""
			},
		}
	}

	fetchPlaylist(id) {
		let self = this
        let request = new Request('/');

        fetch(request).then(function (response) {
            return response
        }).then((result) => {

            const playlist = _.find(playlistData, function(playlist) {
                                                 return playlist.id === id;
			});

            self.setState({
                playlist: playlist,
                id: id
            });
        });
	}

	render() {
		if (this.state.id !== this.props.match.params.id) {
			this.fetchPlaylist(this.props.match.params.id);
		}

		const cuesheetList = this.state.playlist.Cuesheets.map(cuesheet => {
			let url = "/cuesheets/"+cuesheet.id;
			return (<li><a href={url} target="_blank">{cuesheet.title}</a></li>);
		});

		return (
			<div>
				<h2>Playlist - {this.state.playlist.name}</h2>

				<h3>Cuesheet list</h3>

				<ul>
					{cuesheetList}
				</ul>
			</div>
		);
	}
}

const PlaylistWithRouter = withRouter(Playlist);


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
        return (<PlaylistRow playlistId={result.id} name={result.name} />)
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

class PlaylistSearch extends Component {
	constructor() {
            super();
            this.state = {
                playlistResult: [],
                refresh: true
            }
    }

    fetchPlaylists() {
        let self = this
        let request = new Request('/');

        fetch(request).then(function (response) {
            return playlistData;

        }).then((result) => {
            self.setState({
                playlistResult: _.orderBy(result, ['name'], ['asc']),
                refresh: false
            })
        });
    }

	render() {
		if (this.state.refresh) {
			this.fetchPlaylists();
		}

		return (
                        <div>
                            <PlaylistResult searchResult={this.state.playlistResult} />
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

export default PlaylistManager;