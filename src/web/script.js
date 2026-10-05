const find_books_api = 'http://127.0.0.1:3000/books';
const health_api = 'http://127.0.0.1:3000/health';

function display_all() {
    display_books(find_books_api);
}
function display_books(api_to_call) {
    let display_table = document.getElementById("display_results");
    display_table.innerHTML = '';
    fetch(api_to_call, {
        method: 'GET'
    })
        .then(response => {
            if (!response.ok) {
                throw response
            } else {
                return response.json()
            }
        })
        .then(json => {
            let table = "<table id=\"results_table\"></table>";
            $(display_table).append(table);
            let results = document.getElementById("results_table");
            constructTable(results, json)
        })
        .catch(error => {
            if (typeof error.json === "function") {
                error.json().then(jsonError => {
                    console.log("Json error from API");
                    console.log(jsonError);
                }).catch(genericError => {
                    console.log("Generic error from API");
                    console.log(error.statusText);
                });
            } else {
                console.log("Fetch error");
                console.log(error);
            }
        });
}

function check_api() {
    fetch(health_api, {
        method: 'GET'
    })
        .then(response => response.text())
        .then(text => {
            console.log(text);
        })
}

function submitForm() {
    var title = document.getElementById("title");
    var author = document.getElementById("author");
    var isbn = document.getElementById("isbn");
    var rating = document.getElementById("rating");
    var language = document.getElementById("language");
    var genres = document.getElementById("genres");
    var tags = document.getElementById("tags");

    var filters = [title, author, isbn, rating, language, genres, tags];
    var api_call = 'http://127.0.0.1:3000/books?';
    filters.forEach(item => {
        api_call += item.id + "=" + item.value + "&";
    })


    var available = document.getElementsByName("available");
    for (let i = 0; i < available.length; i++) {
        console.log(available[i]);
        if (available[i].checked) {
            switch (available[i].id) {
                case "available_true":
                    api_call += "available=true";
                    break;
                case "available_false":
                    api_call += "available=false";
                    break;
                default:
                    break;
            }
        }
    }

    console.log(api_call);
    display_books(api_call);

    // var allInputs = $(":input");
    // for (var i = 0; i < allInputs.length; i++) {
    //     $('input[type="checkbox"]').prop('checked', false);
    //     $('input').prop('value', '');
    // }
}


function constructTable(selector, list) {

    // Remove existing table content
    $(selector).empty();

    // Get all unique column names
    const columns = [];

    list.forEach(row => {
        for (const key in row) {
            if (!columns.includes(key)) {
                columns.push(key);
            }
        }
    });

    // Create the table header
    const header = $("<tr>");

    columns.forEach(column => {
        if (!(column == "id" || column == "count" || column == "description")) {
            header.append(
                $("<th>").text(capitalizeFirstLetter(column.replace("_", " ")))
            );
        }
    });

    $(selector).append(header);

    // Create table rows
    list.forEach(item => {
        const row = $("<tr>");

        columns.forEach(column => {
            if (!(column == "id" || column == "count" || column == "description")) {

                const value = format_table_data(item, column);

                row.append(
                    $("<td>").text(value)
                );
            }
        });

        $(selector).append(row);
    });
}

function format_table_data(item, column) {
    let value;

    switch (column) {
        case "genres":
        case "tags":

            value = item[column].toString().replace(",", ", ") ?? "";
            break;
        case "available":
            if (item[column] == true) {
                value = "Yes";
            } else {
                value = "No";
            }
            break;
        default:
            value = item[column] ?? "";
            break;
    }

    return value;
}

// Source - https://stackoverflow.com/a/1026087
// Posted by Steve Harrison, modified by community. See post 'Timeline' for change history
// Retrieved 2026-10-04, License - CC BY-SA 4.0

function capitalizeFirstLetter(val) {
    return String(val).charAt(0).toUpperCase() + String(val).slice(1);
}
