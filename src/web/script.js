const find_books_api = 'http://127.0.0.1:3000/books';
const health_api = 'http://127.0.0.1:3000/health';

function search_lib() {
    let results = document.getElementById('results_table');
    let label = document.getElementById('results_label');

    fetch(find_books_api, {
        method: 'GET'
    })
        .then(response => response.json())
        .then(json => {
            console.log(json);
            label.remove();
            constructTable(results, json);
        })


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
        header.append(
            $("<th>").text(column)
        );
    });

    $(selector).append(header);

    // Create table rows
    list.forEach(item => {
        const row = $("<tr>");

        columns.forEach(column => {
            const value =
                item[column] ?? "";

            row.append(
                $("<td>").text(value)
            );
        });

        $(selector).append(row);
    });
}