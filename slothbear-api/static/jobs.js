'use strict';
(function(){
    function checkForApiKey() {
        if (window.localStorage.getItem("slothbear-api-key") === null) {
            window.location.href = "/render";
        }
    }
    function getApiKey() {
        return window.localStorage.getItem("slothbear-api-key");
    }

    function getRenderJobs() {
        var url = "/render/api/";
        var headers = { "X-API-Key": getApiKey() };
        fetch(url, { headers })
            .then((response) => {
                console.log(response.status);
                return response.text();
            })
            .then((data) => {
                console.log(data);
            });
        /*
        var url = "/render/api/jobs?key=" + getApiKey();
        fetch(url)
            .then((response) => {
                console.log(response.status);
                console.log(response.status);
                if (response.ok) {
                    return response.json();                    
                } else {
                    return response.statusText;
                }
            })
            .then((data) => {
                console.log(data);
            });
        */
    }

    function main() {
        checkForApiKey();
        getRenderJobs();
    }

    main();
})();