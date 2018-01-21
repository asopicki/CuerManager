import React, { Component } from 'react';
import { connect } from 'react-redux';
import Modal from 'react-modal';
import {cuesheetSearch, addToListDialog, addToList, closeDialog} from './actions/search';
import {playlistSearch} from './actions/playlists';

import {CUESHEETS_API_PREFIX} from './constants/api'

function SearchButton(props) {

    return (
        <button type="button" name={props.name} alt={props.alt} title={props.alt}
                                        onClick={() => props.onClick()}>{props.name}</button>
    );

};

class SearchRow extends Component {

	constructor(props) {
		super(props)

		this.addToList = this.addToList.bind(this);
	}

	addToList() {
		console.log("Adding to list:", this.props.cuesheetId);
		this.props.addToListHandler(this.props.cuesheetId, this.props.title);
	}

    render() {
        let url = "http://localhost:" + this.props.serverPort + CUESHEETS_API_PREFIX + "/" + this.props.uuid;


	    return  (
	        <tr>
	           <td><a href={url} target="_blank">{this.props.title}</a></td>
	           <td className="textcenter">{this.props.rhythm}</td>
	           <td className="textcenter">{this.props.phase} {this.props.plusfigures}</td>
	           <td className="textcenter" onClick={this.addToList}>addToList</td>
	        </tr>
	    );
	}
};



function SearchResult(props) {

	let searchResult = props.searchResult || [];

    const listRows = searchResult.map(result => {
        return (<SearchRow cuesheetId={result.id} uuid={result.uuid} title={result.title} rhythm={result.rhythm} phase={result.phase}
            plusfigures="" key={result.id} serverPort={props.serverPort}
                addToListHandler={props.addToListHandler} />)
    });

    return (
        <div className="resultList">
            <h2>Search result</h2>
            <table className="textleft">
                <thead>
                    <tr>
                        <td className="textcenter">Title</td>
                        <td className="textcenter">Rhythm</td>
                        <td className="textcenter">Phase</td>
                        <td className="textcenter"></td>
                    </tr>
                </thead>
                <tbody>
                    {listRows}
                </tbody>
            </table>
        </div>
    );
};

class SearchForm extends Component {
    constructor(props) {
        super(props);

        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleSubmit(event) {
        event.preventDefault();

        let search_query = document.getElementById('cuesheet_search').value;

		if ( search_query ) {
            this.props.submitHandler(search_query);
        }
    }

    render() {
        return (<form onSubmit={this.handleSubmit}>
            <label htmlFor="cuesheet_search">Search:</label>
            <input type="text" id="cuesheet_search" name="search" placeholder="Enter search query"
                value={this.props.searchQuery} />

            <button type="submit">Search</button>
        </form>);
    }
}

class AddToListContainer extends Component {
	constructor(props) {
		super(props);

		this.closeDialog = this.closeDialog.bind(this);
	}

	closeDialog(event) {
		let playlistId = document.getElementById('addTitle').value;
        this.props.closeDialog(playlistId, this.props.addTitle.id);
    }

    render() {
        let playlists = this.props.playlists || [];
        let options = playlists.map((playlist) => {
            return (
                <option value={playlist.id} key={playlist.id} >{playlist.name}</option>
            )
        });

        return (
            <Modal isOpen={this.props.showDialog} onRequestClose={this.closeDialog}
                contentLabel="Add title to playlist" className="modalDialog">
                <h2>Add title {this.props.addTitle.title} to the selected Playlist</h2>
                <select id="addTitle">
                    <option value=""></option>
					{options}
                </select>
                <button onClick={this.closeDialog}>Add to list</button>
            </Modal>
        )
    }
}


const mapStateToAddToListProps = state => {
	return {
		showDialog: state.cuesheetSearch.showDialog,
		addTitle: state.cuesheetSearch.addTitle,
		playlists: state.playlists.playlistsResult,
	}
}

const mapDispatchToAddToListProps = dispatch => {
	return {
		closeDialog: (id, titleId) => {
			dispatch(closeDialog());

			if (id) {
				dispatch(addToList(id, titleId))
			}
		}
	}
}

const AddToList = connect(mapStateToAddToListProps, mapDispatchToAddToListProps)(AddToListContainer)

class SearchContainer extends Component {


    handleSearch(query) {
        this.props.searchCuesheet(query);
    }

    handleSearchByPhase(phase) {
        this.handleSearch('phase:'+phase);
    }

    handleSearchByRhythm(rhythm) {
        this.handleSearch('rhythm:'+rhythm);
    }

    render() {
        let self = this;

        let phaseButtons = ['II', 'III', 'IV', 'V', 'VI'].map(phase => {
            let name = 'Phase '+phase;
            let alt = 'Quick search for Phase ' + phase + ' cuesheets';

            return (<SearchButton name={name} alt={alt} key={name}
                        onClick={() => this.handleSearchByPhase(phase)} />)
        });

        let mainRhythmButtons = [ 'Two Step', 'Waltz', 'Cha-Cha-Cha', 'Rumba', 'Foxtrot', 'Tango'].map(rhythm => {
            let name = rhythm;
            let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

            return (<SearchButton name={name} alt={alt} key={name}
                        onClick={() => this.handleSearchByRhythm(rhythm)} />)
        });


        let advancedRhythmButtons = ['Bolero', 'Mambo', 'Quickstep', 'Jive', 'Slow Two Step', 'Samba'].map(rhythm => {
            let name = rhythm;
                    let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

            return (<SearchButton name={name} alt={alt} key={name}
                        onClick={() => this.handleSearchByRhythm(rhythm)} />)
        });

        let additionalRhythmButtons = [ 'Single Swing', 'West Coast Swing', 'Paso Doble', 'Argentine Tango',
                                        'Hesitation Canter Waltz'].map(rhythm => {
            let name = rhythm;
            let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

            return (<SearchButton name={name} alt={alt} key={name}
                        onClick={() => this.handleSearchByRhythm(rhythm)} />)
        });

        return (
            <div>
                <h2>Cuesheet search</h2>
		        <div className="searchInput">
		            <div className="search">
		                <SearchForm submitHandler={(query) => self.handleSearch(query)}/>
		                <div className="searchButtons">
		                    <div className="phaseSearchButtons">{phaseButtons}</div>
		                    <div className="rhythmSearchButtons">{mainRhythmButtons}</div>
		                    <div className="rhythmSearchButtons">{advancedRhythmButtons}</div>
		                    <div className="rhythmSearchButtons">{additionalRhythmButtons}</div>
		                </div>
		            </div>
		        </div>
		        <SearchResult searchResult={this.props.searchResult} serverPort={this.props.serverPort}
		            addToListHandler={this.props.addToListDialog}/>

		        <AddToList />
			</div>
        );
    }
}

const mapStateToProps = state => {
	return {
		searchResult: state.cuesheetSearch.searchResult,
		serverPort: state.cuesheetSearch.serverPort,
		showDialog: state.cuesheetSearch.showDialog,
		addTitle: state.cuesheetSearch.addTitle
	}
}

const mapDispatchToProps = dispatch => {
	return {
		searchCuesheet: query => {
			dispatch(cuesheetSearch(query))
			dispatch(playlistSearch())
		},
		addToListDialog: (id, title) => {
			dispatch(addToListDialog(id, title));
		},
		closeDialog: () => {
			dispatch(closeDialog());
		}
	}
}

const Search = connect(mapStateToProps, mapDispatchToProps)(SearchContainer)

export default Search;