'use strict';
(function(){
    function changeDisplayWhenAuthenticated() {
        // Hide the html elements with the class "hide-if-authenticated"
        var hideElements = document.getElementsByClassName("hide-if-authenticated");
        Array.prototype.forEach.call(hideElements, function(el) {
            el.style.display = "none";
            el.setAttribute("aria-hidden", "true");
        });
        
        // Show the html elements with the class "show-if-authenticated"
        var showElements = document.getElementsByClassName("show-if-authenticated");
        Array.prototype.forEach.call(showElements, function(el){
            el.style.display = "inherit";
            el.setAttribute("aria-hidden", "false");
        });
    }

    function displayErrorMessage(message) {
        console.log(message);
    }

    function saveApiKey(key) {
        window.localStorage.setItem("slothbear-api-key", key);
    }

    function removeApiKey() {
        window.localStorage.removeItem("slothbear-api-key");
        console.log("logged out");
        
    }

    function checkForApiKey() {
        if (window.localStorage.getItem("slothbear-api-key") !== null) {
            changeDisplayWhenAuthenticated();
        }
    }

    function clearUrlParams() {

    }

    function checkForQueryParams() {
        var urlParams = new URLSearchParams(window.location.search);
        if (urlParams.has("authentication")) {
            switch (urlParams.get("authentication")) {
                case "failed":
                    if (urlParams.has("message")) {
                        displayErrorMessage(urlParams.get("message"));
                    }
                    break;
                case "success":
                    if (urlParams.has("key")) {
                        saveApiKey(urlParams.get("key"));
                        window.location.href = "/render";
                    }
                    break;
                default:
                    break;
            }
        }
        if (urlParams.has("logout")) {
            removeApiKey();
            window.location.href = "/render/auth/logout";
        }  
    }

    function main() {
        checkForApiKey();
        checkForQueryParams();
    }

    main();
})();
