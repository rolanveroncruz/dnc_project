import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UploadHighEndServiceFilesComponent } from './upload-high-end-service-files-component';

describe('UploadHighEndServiceFilesComponent', () => {
  let component: UploadHighEndServiceFilesComponent;
  let fixture: ComponentFixture<UploadHighEndServiceFilesComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UploadHighEndServiceFilesComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(UploadHighEndServiceFilesComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
