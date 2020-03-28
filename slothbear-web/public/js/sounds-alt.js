"use strict";
(function(){

    const getSoundData = async () => await fetch('/data/sounds.json')
        .then(response => response.json())
        .then(obj => obj.data);
    /*
    const setupListeners = (searchFunction, watchList) => {
        let searchbox = document.querySelector('#searchbox');
        searchbox.addEventListener('keyup', event => {
            if (event.key === 'Enter') {
                searchFunction(searchbox.value);
            }
        });
        let searchbutton = document.querySelector('#searchbutton');
        searchbutton.addEventListener('click', event => searchFunction(searchbox.value));
    };

    const searchAndDisplay = (fuseFunction, searchString) => {
        const resultTemplate = sound => `
            <article class="ContentBlock, RichText">
                <h4>
                    <a href="${sound.url}" download="${sound.title || sound.keywords}">
                        <i class="fa fa-download"></i>
                        ${sound.title || sound.keywords}
                    </a>
                </h4>
                <p>
                    <strong>Length:</strong> ${sound.length} &nbsp
                    <strong>Genre:</strong> ${sound.genre}
                </p>
                <audio controls>
                    <source src="${sound.url}" type="audio/wav">
                    Your browser does not support the audio element.
                </audio>
            </article>
        `;
        let resultsArray = fuseFunction.search(searchString);
        let results = [];
        resultsArray.forEach(sound => results.push(resultTemplate(sound)));
        let resultsInfo = document.querySelector('#resultsInfo');
        resultsInfo.innerHTML = `Found ${resultsArray.length} sounds for "${searchString}"`
        resultsInfo.style.display = "block";
        let clusterize = new Clusterize({
          rows: results,
          scrollId: 'scrollArea',
          contentId: 'contentArea'
        });
    };
    */

    const main = async () => {
        let data = await getSoundData().then(data => data);
        var options = {
          valueNames: [ 'title', 'keywords', 'length', 'genre', {name: 'url', attr: 'data-src'} ],
          item: '<li><h4 class="title keywords"></h4><p><span class="length"></span> &nbsp<span class="genre"></span></p><audio data-src="" class="url b-lazy" controls></audio></li>',
        };

        let soundList = new List('sounds', options, data);
        /*
        let options = {
            valueNames: [
                'keywords',
                'genre',
                'length'
            ],
            item: `
                <li>
                    <h3 class="keywords"></h3>
                    <p class="length"></p>
                </li>
            `
        }
        let soundList = new List('sound-list', options, soundData);
        /*
        let options = {
            shouldSort: true,
            threshold: 0.6,
            location: 0,
            distance: 100,
            maxPatternLength: 16,
            minMatchCharLength: 3,
            keys: [
                "title",
                "keywords",
                "genre"
            ]
        };
        let fuse = new Fuse(soundData, options);
        let searchFunction = value => searchAndDisplay(fuse, value);
        setupListeners(searchFunction);
        */
    };

    main();
})();
