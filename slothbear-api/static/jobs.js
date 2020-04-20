'use strict';
(function(){
    function redirectToHome() {
        window.location.href = "/render";
    }
    function decodeApiKey(key) {
        // Uses external library import, jwt-decode, https://github.com/auth0/jwt-decode
        return jwt_decode(key);
    }

    function apiKeyNotExpired(decoded_key) {
        // Checks if the api key expiration field is greater than the current time + 2 minutes
        var current_time_in_secs = new Date().getTime() / 1000;
        var current_time_plus_2 = current_time_in_secs + 120;
        return decoded_key.exp > current_time_plus_2;
    }
    function getApiKey() {
        return window.localStorage.getItem("slothbear-api-key");
    }
    function checkForApiKey() {
        var key = getApiKey();
        if (key === null) {
            redirectToHome();
        } else {
            var decoded_key = decodeApiKey(key);
            // if apiKey IS expired (not not)
            if (!apiKeyNotExpired(decoded_key)) {
                redirectToHome();
            }
        }
    }
    
    function getLocalRenderJobs() {
        var renders = window.localStorage.getItem("slothbear-renders");
        if (!renders) {
            renders = "{}";
        }
        renders = JSON.parse(renders);
        return renders;
    }

    function displayRenderJobs(renders) {
        var tableBody = document.getElementById("renderJobsTableBody");
        var ids = Object.keys(renders);
        for (var i = 0; i < ids.length; i++) {
            var render = renders[ids[i]];
            
        }    
    }

    function refreshRenderJobs() {
        var url = "/render/api/";
        var headers = { "X-API-Key": getApiKey() };
        fetch(url, { headers })
            .then((response) => {
                console.log(response.status);
                return response.text();
            })
            .then((data) => {
                //console.log(data);
            });
    }

    function ready(fn) {
        if (document.readyState != 'loading') {
            fn();
        } else {
            document.addEventListener('DOMContentLoaded', fn);
        }
    }

    function main() {
        checkForApiKey();
        var renders = getLocalRenderJobs();
        
        ready(function() { 
            displayRenderJobs(renders);
            refreshRenderJobs();
        });
    }

    main();
})();