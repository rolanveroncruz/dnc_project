import { TestBed } from '@angular/core/testing';

import { SingleDocumentUploadService } from './single-document-upload-service';

describe('SingleDocumentUploadService', () => {
  let service: SingleDocumentUploadService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SingleDocumentUploadService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
