from django.db import models

class RecordTag(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=255)

class Record(models.Model):
    id = models.AutoField(primary_key=True)
    # Vennbase record id
    vennbase_id = models.UUIDField(unique=True)
    name = models.CharField(max_length=255)
    mimetype = models.CharField(max_length=255)
    # array of tags
    tags = models.ManyToManyField(RecordTag, related_name='records')
