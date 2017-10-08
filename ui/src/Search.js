import React, { Component } from 'react';
import _ from 'lodash';

function SearchButton(props) {

    return (
        <button type="button" name={props.name} alt={props.alt} title={props.alt}
                                        onClick={() => props.onClick()}>{props.name}</button>
    );

};

function SearchRow(props) {

    let url = '/cuesheets/' + props.cuesheetId;

    return  (
        <tr>
           <td><a href={url} target="_blank">{props.title}</a></td>
           <td className="textcenter">{props.rhythm}</td>
           <td className="textcenter">{props.phase} {props.plusfigures}</td>
           <td className="textcenter">{props.score}</td>
        </tr>
    );

};

function SearchResult(props) {

    const listRows = props.searchResult.map(result => {
        let score = result.score.toFixed(2);

        return (<SearchRow cuesheetId={result.id} title={result.title} rhythm={result.rhythm} phase={result.phase}
            score={score} plusfigures={result.plusfigures} />)
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

        this.state = {
            searchQuery: ''
        };

        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleChange(event) {
        this.setState({searchQuery: event.target.value});
    }

    handleSubmit(event) {
        event.preventDefault();

        if (this.state.searchQuery) {
            this.props.submitHandler(this.state.searchQuery);
        }
    }

    render() {
        return (<form onSubmit={this.handleSubmit}>
            <label for="search">Search:</label>
            <input type="text" id="search" name="search" placeholder="Enter search query"
                value={this.props.searchQuery} onChange={this.handleChange}/>

            <button type="submit">Search</button>
        </form>);
    }
}

class Search extends Component {
    constructor() {
        super();
        this.state = {
            searchResult: []
        }
    }

    handleSearch(query) {
        let self = this
        let request = new Request('/search/' + query);

        fetch(request).then(function (response) {
            return response.json()
        }).then((result) => {
            self.setState({
                searchResult: _.orderBy(result, ['score', 'phase', 'title'], ['desc', 'asc', 'asc'])
            })
        });
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

            return (<SearchButton name={name} alt={alt}
                        onClick={() => this.handleSearchByPhase(phase)} />)
        });

        let mainRhythmButtons = [ 'Two Step', 'Waltz', 'Cha-Cha-Cha', 'Rumba', 'Foxtrot', 'Tango'].map(rhythm => {
            let name = rhythm;
            let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

            return (<SearchButton name={name} alt={alt}
                        onClick={() => this.handleSearchByRhythm(rhythm)} />)
        });


        let advancedRhythmButtons = ['Bolero', 'Mambo', 'Quickstep', 'Jive', 'Slow Two Step', 'Samba'].map(rhythm => {
            let name = rhythm;
                    let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

            return (<SearchButton name={name} alt={alt}
                        onClick={() => this.handleSearchByRhythm(rhythm)} />)
        });

        let additionalRhythmButtons = [ 'Single Swing', 'West Coast Swing', 'Paso Doble', 'Argentine Tango',
                                        'Hesitation Canter Waltz'].map(rhythm => {
            let name = rhythm;
            let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

            return (<SearchButton name={name} alt={alt}
                        onClick={() => this.handleSearchByRhythm(rhythm)} />)
        });

        return (
            <div>
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
		        <SearchResult searchResult={this.state.searchResult} />
			</div>
        );
    }
}

export default Search;