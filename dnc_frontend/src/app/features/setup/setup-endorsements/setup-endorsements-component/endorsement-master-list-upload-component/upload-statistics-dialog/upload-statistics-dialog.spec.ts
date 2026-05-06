import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UploadStatisticsDialog } from './upload-statistics-dialog';

describe('UploadStatisticsDialog', () => {
  let component: UploadStatisticsDialog;
  let fixture: ComponentFixture<UploadStatisticsDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UploadStatisticsDialog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(UploadStatisticsDialog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
