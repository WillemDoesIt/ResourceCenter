SELECT O.name AS org_name, S.name AS service_name, S.status, 
S.details, S.open_hours, S.is_online_only, S.is_application_only, 
S.eligibility, OrgC.website AS org_website, ServC.website AS service_website,
ServC.email AS service_email, ServC.uri_phone AS service_uri_phone,
L.url_address, L.street, L.unit, L.city, L.state, L.zipcode

FROM Services AS S
JOIN Contacts AS ServC ON ServC.id = S.contact_id
JOIN Orgs AS O ON O.id = S.org_id
JOIN Contacts AS OrgC ON OrgC.id = O.contact_id
JOIN Keywords AS K ON S.keyword_id = K.id
JOIN Locations AS L ON S.location_id = L.id

WHERE O.name LIKE '%%' 
OR S.name LIKE '%%'
OR S.details LIKE '%%'
OR O.description LIKE '%%'
OR K.service_type LIKE '%%'
OR K.quick_filter LIKE '%%'
OR K.service_groups LIKE '%%'
OR K.tags LIKE '%%'
OR K.tag_groups LIKE '%%';