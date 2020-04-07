'use strict';
// Create the main myMSALObj instance
// configuration parameters are located at authConfig.js
var myMSALObj = new Msal.UserAgentApplication(msalConfig);

var accessToken;

// Register Callbacks for Redirect flow
myMSALObj.handleRedirectCallback(authRedirectCallBack);

function authRedirectCallBack(error, response) {
    if (error) {
        console.log(error);
    } else {
        if (response.tokenType === "id_token") {
            console.log('id_token acquired at: ' + new Date().toString());        
        } else if (response.tokenType === "access_token") {
            console.log('access_token acquired at: ' + new Date().toString());
            accessToken = response.accessToken;

            changeAuthenticatedDisplay();
        } else {
            console.log("token type is:" + response.tokenType);
        }
    }
}

function logIn() {
    myMSALObj.loginRedirect(loginRequest);
}

function logOut() {
    myMSALObj.logout();
}

function changeAuthenticatedDisplay() {
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

