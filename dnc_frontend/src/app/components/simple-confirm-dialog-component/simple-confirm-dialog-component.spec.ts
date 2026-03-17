import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SimpleConfirmDialogComponent } from './simple-confirm-dialog-component';

describe('SimpleConfirmDialogComponent', () => {
  let component: SimpleConfirmDialogComponent;
  let fixture: ComponentFixture<SimpleConfirmDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SimpleConfirmDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SimpleConfirmDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
