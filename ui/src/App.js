import React, { Component } from 'react';
import './App.css';

function SearchButton(props) {

    return (
        <button type="button" name={props.name} alt={props.alt} title={props.alt}
                                        onClick={() => props.onClick()}>{props.name}</button>
    );

};

function SearchRow(props) {

    return  (
        <tr>
           <td>{props.title}</td>
           <td>{props.rhythm}</td>
           <td>{props.phase}</td>
        </tr>
    );

};

function SearchResult(props) {

    const listRows = props.searchResult.map(result =>
            <SearchRow title={result.title} rhythm={result.rhythm} phase={result.phase} />);

    return (
        <div className="resultList">
            <h2>Search result</h2>
            <table>
                <thead>
                    <tr>
                        <td>Title</td>
                        <td>Rhythm</td>
                        <td>Phase</td>
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

class App extends Component {

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
                searchResult: result
            })
        });
    }

    handleSearchByPhase(phase) {
        this.handleSearch('phase/'+phase);
    }

    render() {
        let self = this;

        let phaseButtons = ['II', 'III', 'IV', 'V', 'VI'].map(phase => {
            let name = 'Phase '+phase;
            let alt = 'Quick search for Phase ' + phase + ' cuesheets';

            return (<SearchButton name={name} alt={alt}
                        onClick={() => this.handleSearchByPhase(phase)} />)
        });

        let rhythmButtons = [ 'Two Step', 'Waltz', 'Cha-Cha-Cha', 'Rumba', 'Foxtrot', 'Tango', 'Bolero', 'Mambo',
            'Quickstep', 'Jive', 'Slow Two Step', 'Samba', 'Paso Doble', 'Single Swing', 'West Coast Swing',
            'Argentine Tango', 'Hesitation Canter Waltz'].map(rhythm => {
                    let name = rhythm;
                    let alt = 'Quick search for cuesheets for the rhythm '+ rhythm;

                    return (<SearchButton name={name} alt={alt}
                                onClick={() => this.handleSearch(rhythm)} />)
        });

        return (
            <div className="App">
                <div className="searchInput">
                    <h1>Cueing Manager - Cuesheet list</h1>
                    <div className="search">
                        <SearchForm submitHandler={(query) => self.handleSearch(query)}/>
                        <div className="searchButtons">
                            <div>{phaseButtons}</div>
                            <div>{rhythmButtons}</div>
                        </div>
                    </div>
                </div>
                <SearchResult searchResult={this.state.searchResult} />
            </div>
        );
    }
}

export default App;
