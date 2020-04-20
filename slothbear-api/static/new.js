'use strict';
(function(){
    var RENDER_DATA_SET = {};

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
    
    function saveRenderJobData(data) {
        var renders = window.localStorage.getItem("slothbear-renders");
        if (!renders) {
            renders = {};
        }
        renders[data.id] = data;
    }

    function addDataToRender(key, data) {
        // Takes the render dataset (dictionary/object)
        // and adds the passed data to it accessible by the passed key
        RENDER_DATA_SET[key] = data;
    }

    function postNewRender() {
        // convert the frame sizes to numbers
        RENDER_DATA_SET.frameWidth = parseInt(RENDER_DATA_SET.frameWidth, 10);
        RENDER_DATA_SET.frameHeight = parseInt(RENDER_DATA_SET.frameHeight, 10);
        // Send POST request to API
        fetch('/render/api/job', { 
            headers: { 
                'X-API-Key': getApiKey(),
                'Content-Type': 'application/json',
            },
            method: 'POST',
            body: JSON.stringify(RENDER_DATA_SET),
        })
        .then((response) => {
            if (response.ok) {
                return response.json();
            } else {
                return response.text();
            }
        })
        .then((data) => {
            if (data.id) {
                // response is ok, data is json
                saveRenderJobData(data);
                displayPageNotice("Success", 'Job was submitted successfully. <a href="/render/jobs.html">Go to Your Render Jobs</a>');
                
                //displayYourRenderJobsButton();
            } else {
                if (data) {
                    // response is not ok, data is a string
                    var domparser = new DOMParser();
                    var html = domparser.parseFromString(data, "text/html");
                    if (html.body) {
                        // response is html
                        var h1 = html.querySelector('h1');
                        var p = html.querySelector('p');
                        displayPageNotice("Error: " + h1.innerHTML, p.innerHTML);
                    } else {
                        // response is not html
                        displayPageNotice("Error:", data);
                    }
                } else {
                    // repsonse in not ok, no data returned
                    displayPageNotice("Error:", "There was an error processing your request.");
                }
            }
        });
    }

    function displayPageNotice(title, message, timeout=10) {
        var noticeBlock = document.getElementById("notice_block");
        var noticeTitle = document.getElementById("notice_title");
        var noticeMessage = document.getElementById("notice_message");
        // display the notice block
        noticeBlock.setAttribute("aria-label", title + " Alert");
        noticeBlock.setAttribute("role", "alert");
        noticeBlock.style.display = "inherit";
        noticeBlock.setAttribute("aria-hidden", "false");
        // set the title and message
        noticeTitle.innerHTML = title;
        noticeMessage.innerHTML = message;
        // fade out the notice block
        setTimeout(function(){  
            noticeBlock.style.opacity = '0';
            noticeBlock.style.display = "none";
            noticeBlock.setAttribute("aria-hidden", "true");
        }, timeout * 1000);
    }

    function showInputErrorMessage(input, errorMessage) {
        
        // Add error class to input
        input.classList.add('error');

        // Get input id or name
        var id = input.id || input.name;
        if (!id) return;

        // Check if error message input already exists
        // If not, create one
        var message = input.form.querySelector('.error-message#error-for-' + id );
        if (!message) {
            message = document.createElement('div');
            message.className = 'error-message';
            message.id = 'error-for-' + id;
            input.parentNode.insertBefore( message, input.nextSibling );
        }

        // Add ARIA role to the field
        input.setAttribute('aria-describedby', 'error-for-' + id);

        // Update error message
        message.innerHTML = errorMessage;

        // Show error message
        message.style.display = 'block';
        message.style.visibility = 'visible';
    }

    function removeAnyErrorsFromInput(input) {
        // Remove error class to input
        input.classList.remove('error');

        // Remove ARIA role from the input
        input.removeAttribute('aria-describedby');

        // Get input id or name
        var id = input.id || input.name;
        if (!id) return;

        // Check if an error message is in the DOM
        var message = input.form.querySelector('.error-message#error-for-' + id + '');
        if (!message) return;

        // If so, hide it
        message.innerHTML = '';
        message.style.display = 'none';
        message.style.visibility = 'hidden';
        message.setAttribute("aria-hidden", "true");
    }
     
    function inputErrorCheck(input) {
        // don't validate file inputs
        if (input.type === "file") return null;

        // get validity
        var validity = input.validity;
        
        // If valid, return null
        if (validity.valid) return null;

        // If input is required and empty
        if (validity.valueMissing) return "Please fill out this field.";

         // If number input isn't a number
        if (validity.badInput) return "Please enter a number.";

        // If a number input is over the max
        if (validity.rangeOverflow) {
            var message = "Please select a value that is no more than " + input.getAttribute('max') + ".";
            return message;
        }

        // If all else fails, return a generic catchall error
        return "The value you entered for this field is invalid.";
    }

    function addEventListenerToInputs(event, fn) {
        // adds event listeners to form input fields
        // gets the input elements from the DOM
        var inputs = document.getElementsByClassName('listen');
        // loops through the set of inputs
        Array.prototype.forEach.call(inputs, function(input, i) {
            // adds an event listener to each input
            // registers the passed function, passing it the input argument
            input.addEventListener(event, (e) => fn(e, input));
        });
    }

    function listenForInputChanges() {
        addEventListenerToInputs('change', (e, input) => addDataToRender(input.id, input.value));
    }
    function listenForProjectDirectoryChanges() {
        var pathProject = document.getElementById("pathProject");
        var pathScene = document.getElementById("pathScene");
        var pathOutput = document.getElementById("pathOutput");
        pathProject.addEventListener('change', (e) => {
            var projectDir = e.target.value;
            if (!pathScene.value) {
                pathScene.value = projectDir;
            }
            if (!pathOutput.value) {
                pathOutput.value = projectDir;
            }
        });
    }

    function listenForFormValidation() {
        var form = document.getElementById("newRenderJobForm");
        form.setAttribute('novalidate', true);
        addEventListenerToInputs('blur', (e, input) => {
            var errorMessage = inputErrorCheck(input);
            if (errorMessage) {
                showInputErrorMessage(input, errorMessage);
            } else {
                removeAnyErrorsFromInput(input);
            }
        });
    }

    function listenForSubmitClick(submitFn) {
        // add event listener to the form's submit button
        var submitBtn = document.getElementById('submitBtn');
        // When the Submit button is clicked, this event will call the submitFn
        submitBtn.addEventListener('click', submitFn);
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
        
        // Create the function to call
        // when the Submit button is clicked
        function submitFn() {
            // Check inputs for errors
            var inputs = document.getElementsByClassName('listen');
            var errorMessage;
            var hasError;

            Array.prototype.forEach.call(inputs, function(input, i) {
                errorMessage = inputErrorCheck(input);
                if (errorMessage) {
                    showInputErrorMessage(input, errorMessage);
                    if (!hasError) {
                        hasError = input;
                    }
                }
            });
            if (hasError) {
                hasError.focus();
                displayPageNotice("Error", "Please correct the form's errors before Submitting");
            } else {
                // No errors, so submit the form
                postNewRender();
            }
        }

        ready(function() { 
            listenForInputChanges();
            listenForProjectDirectoryChanges();
            listenForFormValidation();
            listenForSubmitClick(submitFn);
        });
    }

    main();
})();