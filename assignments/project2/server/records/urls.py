from django.urls import path
from records.views import retrieve_record, RecordViewSet

urlpatterns = [
    path('', RecordViewSet.as_view({ 'get': 'list', 'post': 'create' }), name='record-list'),
    path('query', RecordViewSet.as_view({ 'post': 'retrieve' }), name='record-query'),
    # Retrievers a record from Vennbase with its corresponding content-type
    path('<str:vennbase_id>', retrieve_record, name='record-retrieve')
]
