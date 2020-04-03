'use strict';
(function(){
    //console.log("localStorage test:", window.localStorage.getItem("test"));
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
    //changeDisplayWhenAuthenticated();
})();