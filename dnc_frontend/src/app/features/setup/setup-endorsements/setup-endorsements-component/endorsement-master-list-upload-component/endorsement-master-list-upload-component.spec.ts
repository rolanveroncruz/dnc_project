import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EndorsementMasterListUploadComponent } from './endorsement-master-list-upload-component';

describe('EndorsementMasterListUploadComponent', () => {
  let component: EndorsementMasterListUploadComponent;
  let fixture: ComponentFixture<EndorsementMasterListUploadComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [EndorsementMasterListUploadComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(EndorsementMasterListUploadComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
