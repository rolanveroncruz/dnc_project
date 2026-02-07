import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SingleDocumentSlotComponent } from './single-document-slot-component';

describe('SingleDocumentSlotComponent', () => {
  let component: SingleDocumentSlotComponent;
  let fixture: ComponentFixture<SingleDocumentSlotComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SingleDocumentSlotComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SingleDocumentSlotComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
