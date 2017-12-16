import React, { Component } from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router-dom'
import {cuesheetSearch} from './actions/search';

import {CUESHEETS_API_PREFIX} from './constants/api'

function SearchButton(props) {

    return (
        <button type="button" name={props.name} alt={props.alt} title={props.alt}
                                        onClick={() => props.onClick()}>{props.name}</button>
    );

};

function SearchRow(props) {

    let url = "http://localhost:" + props.serverPort + CUESHEETS_API_PREFIX + "/" + props.cuesheetId; //TODO: Move url prefix to state or constant

    return  (
        <tr>
           <td><a href={url} target="_blank">{props.title}</a></td>
           <td className="textcenter">{props.rhythm}</td>
           <td className="textcenter">{props.phase} {props.plusfigures}</td>
           <td className="textcenter">{props.score}</td>
           <td className="textcenter">addToList</td>
        </tr>
    );

};

function SearchResult(props) {

	let searchResult = props.searchResult || [];

    const listRows = searchResult.map(result => {
        let score = result.score.toFixed(2);

        return (<SearchRow cuesheetId={result.id} title={result.title} rhythm={result.rhythm} phase={result.phase}
            score={score} plusfigures={result.plusfigures} key={result.id} serverPort={props.serverPort} />)
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
                        <td className="textcenter">Score</td>
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
		        <SearchResult searchResult={this.props.searchResult} serverPort={this.props.serverPort}/>
			</div>
        );
    }
}

const mapStateToProps = state => {
	return {
		searchResult: state.cuesheetSearch.searchResult,
		serverPort: state.cuesheetSearch.serverPort
	}
}

const mapDispatchToProps = dispatch => {
	return {
		searchCuesheet: query => {
			dispatch(cuesheetSearch(query))
		}
	}
}

const Search = connect(mapStateToProps, mapDispatchToProps)(SearchContainer)

export default Search;