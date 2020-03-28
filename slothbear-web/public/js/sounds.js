"use strict";
(function(){

    const getSoundData = async () => await fetch('/data/sounds.json')
        .then(response => response.json())
        .then(obj => obj.data);

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
                <button class="preview" data-url="${sound.url}" data-name="${sound.title || sound.keywords}">Preview</button>
            </article>
        `;
        let player = document.querySelector('#player');
        let playerName = document.querySelector('#playerName');
        playerName.innerHTML = '';
        player.pause();
        player.src = '';
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
        let previewButtons = document.querySelectorAll('.preview');
        previewButtons.forEach(button => {
            button.addEventListener('click', event => {
                let url = event.target.dataset.url;
                let name = event.target.dataset.name;
                let player = document.querySelector('#player');
                let playerName = document.querySelector('#playerName');
                player.src = url;
                playerName.innerHTML = `Now previewing ${name}`;
                player.play();
            });
        });
    };

    const main = async () => {
        let soundData = await getSoundData().then(data => data);
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
    };

    main();
})();
